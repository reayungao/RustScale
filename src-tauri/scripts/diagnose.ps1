# diagnose.ps1
# Captures full cargo check output to a file for AI analysis.

$ErrorActionPreference = "Continue"
$LogFile = "../compilation_log.txt"

Write-Host "Running cargo check and capturing output to $LogFile..."

# Run cargo check and redirect both stdout and stderr to the log file
# We use cmd /c to ensure proper redirection in all environments
cmd /c "cargo check --color always 2>&1" | Out-File -Encoding UTF8 $LogFile

# Check if the command failed
if ($LASTEXITCODE -ne 0) {
    Write-Host "Compilation FAILED. Log saved to $LogFile"
    Write-Host "--- TAIL OF LOG (Last 20 lines) ---"
    Get-Content $LogFile -Tail 20
    exit 1
} else {
    Write-Host "Compilation SUCCEEDED."
    exit 0
}
