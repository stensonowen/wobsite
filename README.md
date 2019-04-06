### site

### Dirs
`/var/www/oms.sh/`
* `static`
* `public`

`/etc/nginx/`
* `sites-available/oms.sh`
* `sites-enabled/oms.sh`

### Setup
```sh
mkdir -p /var/www/oms.sh/public
cp -r static /var/www/oms.sh/

cp oms.sh /etc/nginx/sites-available/oms.sh
ln -s /etc/nginx/sites-available/oms.sh /etc/nginx/sites-enabled/oms.sh
```

