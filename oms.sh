# Virtual Host configuration for oms.sh
#  /etc/nginx/sites-enabled/oms.sh -> /etc/nginx/sites-available/oms.sh

server {
    listen 8080 default_server;
    listen [::]:8080 default_server;

    root /var/www/oms.sh/static;

    index work.html;

    server_name oms.sh www.oms.sh;

    error_page 404 /404.png;

    location / {
        # optionally ignore trailing /
        try_files $uri $uri/ $uri.html =404;
    }

    location /files/ {
        alias /var/www/oms.sh/public/;
        autoindex off;
    }

}

