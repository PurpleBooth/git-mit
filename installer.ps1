# git-mit installer script for Windows

# Define architecture
$Arch = "x86_64-pc-windows-msvc.exe"

# Define binaries to download
$Binaries = @("git-mit", "git-mit-config", "git-mit-install", "git-mit-relates-to", "mit-commit-msg", "mit-pre-commit", "mit-prepare-commit-msg")

# Create install directory if it doesn't exist
$InstallDir = "$env:USERPROFILE\git-mit"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

foreach ($Binary in $Binaries)
{
    Write-Host "📥 Downloading $Binary..."
    $BinaryUrl = "https://github.com/PurpleBooth/git-mit/releases/latest/download/${Binary}-${Arch}"
    $HashUrl = "${BinaryUrl}.sha256"

    # Download files
    Invoke-WebRequest -Uri $BinaryUrl -OutFile "$Binary.exe"
    Invoke-WebRequest -Uri $HashUrl -OutFile "$Binary.sha256"

    # Verify SHA256
    $ExpectedHash = Get-Content "$Binary.sha256" -Raw
    $ActualHash = (Get-FileHash "$Binary.exe" -Algorithm SHA256).Hash.ToLower()

    if ($ActualHash -eq $ExpectedHash.Split()[0].ToLower())
    {
        Write-Host "✅ Verified $Binary"
        Move-Item "$Binary.exe" "$InstallDir" -Force
    }
    else
    {
        Write-Host "❌ Verification failed for $Binary"
        exit 1
    }
}

# Add to PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*")
{
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
}

Set-Location -

Write-Host "🎉 Installation complete! Please restart your terminal and run 'git mit-install' to set up your repository."
