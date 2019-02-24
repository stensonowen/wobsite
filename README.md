### site

# Setup
```sh
mkdir -p /var/www/oms.sh/public
cp -r static /var/www/oms.sh/  # ln instead? glob?

cp nginx.conf /etc/nginx/sites-available/oms.sh
ln -s /etc/nginx/sites-available/oms.sh /etc/nginx/sites-enabled/oms.sh
```

