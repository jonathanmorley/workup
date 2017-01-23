$ErrorActionPreference = "Stop"

If (!([Security.Principal.WindowsPrincipal] `
     [Security.Principal.WindowsIdentity]::GetCurrent()
    ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
  Write-Error "You do not have Administrator rights to run this script!`nPlease re-run this script as an Administrator!"
}

Function Reset-Path {
  $MachinePaths = [Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::Machine) -split ';'
  $UserPaths = [Environment]::GetEnvironmentVariable('Path', [System.EnvironmentVariableTarget]::User) -split ';'
  $Env:Path = ($MachinePaths + $UserPaths) -join ';'
}

Function Invoke-WrappedCommand([string] $action, $block) {
  Write-Host -NoNewLine "${action}... "
  Try {
    & $block | Out-Null
    Write-Host -ForegroundColor 'Green' 'OK'
  } Catch {
    Write-Host -ForegroundColor 'Red' "Error ($($Error[0].Exception))"
  }
}

$WORKUP_VERSION = "0.1.6"
$WORKUP_URL = "https://github.com/cvent/workup/releases/download/v${WORKUP_VERSION}/workup.msi"
$WORKUP_DIR = Join-Path ${HOME} '.workup'

Get-WmiObject -Class Win32_Product -Filter "Name LIKE 'Workup%'" |% {
  Invoke-WrappedCommand "Uninstalling Workup v$($_.Version)" { $_.Uninstall() }
}

If (!(Test-Path ${WORKUP_DIR} -PathType 'Container')) {
  Invoke-WrappedCommand "Creating ~/.workup directory" {
    New-Item -Type Directory ${WORKUP_DIR}
  }
}

Invoke-WrappedCommand "Installing Workup v${WORKUP_VERSION}" {
  $installer = Join-Path $WORKUP_DIR 'workup.msi'
  (New-Object System.Net.WebClient).DownloadFile($WORKUP_URL, $installer)
  cmd /c start '' /wait msiexec /i $installer /qn
}

Reset-Path

Invoke-WrappedCommand "Checking for workup command" { Get-Command 'workup' }

Write-Host 'You are ready to run workup'
