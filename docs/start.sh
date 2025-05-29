tsc main.ts > /dev/null 2>&1
echo 'Compiling TS'
sass style.scss style.css > /dev/null 2>&1
echo 'Compiling SCSS'
echo 'Running Web Server'
read -p '1 PYWS, 2. PHPWS :' server
if [ $server == "1" ];
then
    python3 -m http.server 8888
elif [ $server == "2" ];
then
    php -S localhost:8888
fi
echo 'http://localhost:8888'
