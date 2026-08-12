Start-Sleep -Seconds 4
$r = Invoke-WebRequest -Uri http://localhost:3000 -UseBasicParsing
Write-Output "STATUS: $($r.StatusCode)"
if ($r.Content -match 'rotunda-center') { Write-Output "rotunda-center: PRESENT" } else { Write-Output "rotunda-center: NOT FOUND" }
if ($r.Content -match 'GRAND ROTUNDA') { Write-Output "GRAND ROTUNDA: PRESENT" } else { Write-Output "GRAND ROTUNDA: NOT FOUND" }
if ($r.Content -match 'alcoves') { Write-Output "alcoves: PRESENT" } else { Write-Output "alcoves: NOT FOUND" }
