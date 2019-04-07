# Virtual Host configuration for oms.sh
#  /etc/nginx/sites-enabled/oms.sh -> /etc/nginx/sites-available/oms.sh

server {
    listen 443 ssl;

    server_name oms.sh www.oms.sh;


    root /var/www/oms.sh/static;
    index work.html;
    error_page 404 /404.png;


    ssl_certificate /etc/letsencrypt/live/oms.sh/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/oms.sh/privkey.pem;

    ssl_protocols TLSv1 TLSv1.1 TLSv1.2;
    ssl_prefer_server_ciphers on;
    ssl_dhparam /etc/ssl/certs/dhparam.pem;
    ssl_ciphers 'ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-DSS-AES128-GCM-SHA256:kEDH+AESGCM:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA:ECDHE-ECDSA-AES256-SHA:DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:DHE-DSS-AES128-SHA256:DHE-RSA-AES256-SHA256:DHE-DSS-AES256-SHA:DHE-RSA-AES256-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:AES:CAMELLIA:DES-CBC3-SHA:!aNULL:!eNULL:!EXPORT:!DES:!RC4:!MD5:!PSK:!aECDH:!EDH-DSS-DES-CBC3-SHA:!EDH-RSA-DES-CBC3-SHA:!KRB5-DES-CBC3-SHA';
    ssl_session_timeout 1d;
    ssl_session_cache shared:SSL:50m;
    ssl_stapling on;
    ssl_stapling_verify on;
    add_header Strict-Transport-Security max-age=15768000;
    add_header Content-Security-Policy "default-src 'none'; img-src 'self'; script-src 'none'; style-src 'self'; object-src 'none'; report-uri /csp-reports; frame-ancestors 'none'";
    add_header X-Frame-Options DENY;


    location / {
        # optionally ignore trailing /
        try_files $uri $uri/ $uri.html =404;
    }

    location /files/ {
        alias /var/www/oms.sh/public/;
        autoindex off;
    }

    location /.well-known/ {
        autoindex off;
    }

}


server {
    listen 80;

    server_name oms.sh www.oms.sh;

    return 301 https://$host$request_uri;
}

