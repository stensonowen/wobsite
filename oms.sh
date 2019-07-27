# Virtual Host configuration for oms.sh
#  /etc/nginx/sites-enabled/oms.sh -> /etc/nginx/sites-available/oms.sh

server {

    server_name oms.sh www.oms.sh;

    root /var/www/oms.sh/static;
    index work.html;
    error_page 404 /404.png;

    location / {
        # optionally ignore trailing /
        try_files $uri $uri/ $uri.html =404;
    }

    location /files/ {
        alias /var/www/oms.sh/public/;
        autoindex off;
    }

    listen 443 ssl; # managed by Certbot
    ssl_certificate /etc/letsencrypt/live/oms.sh/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/oms.sh/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot


}

server {
    listen 80;
    server_name old.oms.sh;
    location / {
        proxy_pass http://45.76.30.199;
    }
}

server {
    if ($host = www.oms.sh) {
        return 301 https://$host$request_uri;
    } # managed by Certbot


    if ($host = oms.sh) {
        return 301 https://$host$request_uri;
    } # managed by Certbot


    listen 80;

    server_name oms.sh www.oms.sh;
    return 404; # managed by Certbot




}
