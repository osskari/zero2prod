if (-not (Get-Command psql.exe -errorAction SilentlyContinue)) {
    Write-Output "Error: psql is not installed";
    exit 1;
}

if (-not (Get-Command sqlx.exe -errorAction SilentlyContinue)) {
    Write-Output "Error: sqlx is not installed";
    Write-Output "Use:";
    Write-Output "  cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,postgres";
    Write-Output "to install it";
    exit 1;
}

New-Variable -Name DB_USER -Value $POSTGRES_USER??"postgres";
New-Variable -Name DB_PASSWORD -Value $POSTGRES_PASSWORD??"password";
New-Variable -Name DB_NAME -Value $POSTGRES_DB??"newsletter";
New-Variable -Name DB_PORT -Value $POSTGRES_PORT??"5432";
New-Variable -Name DB_HOST~ -Value $POSTGRES_HOST??"localhost";

docker.exe run -e POSTGRES_USER=${DB_USER} -e POSTGRES_PASSWORD=${DB_PASSWORD} -e POSTGRES_DB=${DB_NAME} -p ${DB_PORT}:5432 -d postgres postgres -N 1000;
#
#$env:PGPASSWORD="${DB_PASSWORD}";
#
#while ($true) {
#    psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q';
#}