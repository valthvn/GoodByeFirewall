# Configure only an installed, protected executable. Run from an elevated shell.
$ErrorActionPreference = 'Stop'
$identity = [System.Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object System.Security.Principal.WindowsPrincipal($identity)
if (-not $principal.IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'Relancez ce script depuis PowerShell en mode administrateur.'
}

function Assert-ProtectedPath([string] $Path) {
    $acl = Get-Acl -LiteralPath $Path
    $descriptor = New-Object System.Security.AccessControl.RawSecurityDescriptor($acl.GetSecurityDescriptorSddlForm('All'))
    if ($null -eq $descriptor.DiscretionaryAcl) { throw "Droits non sécurisés : $Path" }
    $privileged = @('S-1-5-18', 'S-1-5-32-544', 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464')
    $owner = $acl.GetOwner([System.Security.Principal.SecurityIdentifier]).Value
    if ($owner -notin $privileged) { throw "Propriétaire non sécurisé : $Path" }
    foreach ($rule in $acl.GetAccessRules($true, $true, [System.Security.Principal.SecurityIdentifier])) {
        if ($rule.AccessControlType -ne 'Allow' -or ($rule.PropagationFlags -band [System.Security.AccessControl.PropagationFlags]::InheritOnly)) { continue }
        if (([long] $rule.FileSystemRights -band 0x500d0156) -and $rule.IdentityReference.Value -notin $privileged) {
            throw "Installation modifiable par un utilisateur non administrateur : $Path"
        }
    }
}

$appDir = Join-Path $env:ProgramFiles 'GoodByeFirewall'
$exePath = (Get-Item -LiteralPath (Join-Path $appDir 'GoodByeFirewall.exe')).FullName
Assert-ProtectedPath $appDir
Assert-ProtectedPath $exePath

$task = Get-ScheduledTask -TaskName 'GoodByeFirewall' -ErrorAction SilentlyContinue
if ($task -and (@($task.Actions).Count -ne 1 -or $task.Actions[0].Execute -ne $exePath -or $task.Actions[0].Arguments -ne '--no-task-elevate')) {
    throw 'Une autre installation possède déjà la tâche GoodByeFirewall. Aucune modification effectuée.'
}
$action = New-ScheduledTaskAction -Execute $exePath -Argument '--no-task-elevate'
$taskPrincipal = New-ScheduledTaskPrincipal -UserId $identity.Name -LogonType Interactive -RunLevel Highest
$settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Days 0)
Register-ScheduledTask -TaskName 'GoodByeFirewall' -Action $action -Principal $taskPrincipal -Settings $settings -Force | Out-Null
Write-Host 'Tâche GoodByeFirewall configurée pour cette installation protégée.' -ForegroundColor Green

# Raccourcis bureau et menu démarrer
$iconPath = Join-Path $appDir "AppIcon.ico"
if (-not (Test-Path $iconPath)) {
    $iconPath = Join-Path $appDir "assets\AppIcon.ico"
}

$wsh = New-Object -ComObject WScript.Shell

$desktopDir = [Environment]::GetFolderPath('Desktop')
$desktopLnk = Join-Path $desktopDir "GoodByeFirewall.lnk"
if (Test-Path $desktopLnk) {
    $sc = $wsh.CreateShortcut($desktopLnk)
    $sc.TargetPath = $exePath
    $sc.Arguments = ""
    $sc.WorkingDirectory = $appDir
    if (Test-Path $iconPath) {
        $sc.IconLocation = "$iconPath,0"
    }
    $sc.Description = "GoodByeFirewall"
    $sc.Save()
}

$startMenuDir = [Environment]::GetFolderPath('StartMenu')
$startMenuLnk = Join-Path $startMenuDir "Programs\GoodByeFirewall.lnk"
if (Test-Path (Split-Path $startMenuLnk)) {
    $sc2 = $wsh.CreateShortcut($startMenuLnk)
    $sc2.TargetPath = $exePath
    $sc2.Arguments = ""
    $sc2.WorkingDirectory = $appDir
    if (Test-Path $iconPath) {
        $sc2.IconLocation = "$iconPath,0"
    }
    $sc2.Description = "GoodByeFirewall"
    $sc2.Save()
}

# Enregistrement notifications Windows
$regAumid = "HKCU:\Software\Classes\AppUserModelId\GoodByeFirewall"
if (-not (Test-Path $regAumid)) { New-Item -Path $regAumid -Force | Out-Null }
Set-ItemProperty -Path $regAumid -Name "DisplayName" -Value "GoodByeFirewall" -Force
Set-ItemProperty -Path $regAumid -Name "IconUri" -Value $iconPath -Force
Set-ItemProperty -Path $regAumid -Name "ShowInSettings" -Value 1 -Type DWord -Force

# Rafraîchissement icônes shell
if (-not ('ShellNotify' -as [type])) {
Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class ShellNotify {
    [DllImport("shell32.dll")]
    public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);
}
"@
}
[ShellNotify]::SHChangeNotify(0x08000000, 0, [IntPtr]::Zero, [IntPtr]::Zero)

Write-Host "Raccourcis et configuration GoodByeFirewall rétablis avec succès." -ForegroundColor Green
