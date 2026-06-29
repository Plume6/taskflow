Write-Host "Testing RustyExpress API..." -ForegroundColor Cyan

Write-Host "`n1. Testing health check..." -ForegroundColor Yellow
try {
    $result = Invoke-RestMethod -Uri "http://127.0.0.1:8080/health" -Method Get
    Write-Host "✓ Health check passed: $($result.data.status)" -ForegroundColor Green
} catch {
    Write-Host "✗ Health check failed: $_" -ForegroundColor Red
    Write-Host "  Make sure backend is running on port 8080" -ForegroundColor Red
    exit 1
}

Write-Host "`n2. Testing register..." -ForegroundColor Yellow
try {
    $body = @{username="testuser"; email="test@example.com"; password="123456"} | ConvertTo-Json
    $result = Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/auth/register" -Method Post -ContentType "application/json" -Body $body
    Write-Host "✓ Register success: $($result.username)" -ForegroundColor Green
} catch {
    Write-Host "Register error (may already exist): $_" -ForegroundColor Gray
}

Write-Host "`n3. Testing login..." -ForegroundColor Yellow
try {
    $body = @{email="test@example.com"; password="123456"} | ConvertTo-Json
    $result = Invoke-RestMethod -Uri "http://127.0.0.1:8080/api/v1/auth/login" -Method Post -ContentType "application/json" -Body $body
    Write-Host "✓ Login success! Token: $($result.token.Substring(0, 20))..." -ForegroundColor Green
} catch {
    Write-Host "✗ Login failed: $_" -ForegroundColor Red
}

Write-Host "`nDone!" -ForegroundColor Cyan