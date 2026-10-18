# Setup WordPress (for Fedora or CentOS9)

Use root user for instalation

```
dnf install  wordpress
yum install mariadb-server
systemctl enable mariadb
mariadb
```

Now you are in the SQL Shell.
Type the following SQL Commands.

```
show databases;
CREATE DATABASE wordpress;
CREATE USER 'wordpress'   IDENTIFIED BY 'your_mmm_password';
GRANT ALL PRIVILEGES ON wordpress.*  TO 'wordpress';
quit;
```

### Check the database connectivity via TCP/IP

```
[…]$ mysql -u wordpress wordpress  -p
Enter password:
Welcome to the MariaDB monitor.  Commands end with ; or \g.
Your MariaDB connection id is 18
Server version: 10.5.20-MariaDB MariaDB Server

Copyright (c) 2000, 2018, Oracle, MariaDB Corporation Ab and others.

Type 'help;' or '\h' for help. Type '\c' to clear the current input statement.‘

MariaDB [wordpress]> quit
[…]$
```

### Edit the Wordpress configuration file which is in Fedora stored at /etc/wordpress (to achieve full Fedora integration).

```
[…]# vim /etc/wordpress/wp-config.php
define( 'DB_NAME', 'wordpress' );

/** Database username */
define( 'DB_USER', 'wordpress' );

/** Database password */
define( 'DB_PASSWORD', 'your_mmm_password' );

/** Database hostname */
define( 'DB_HOST', 'localhost' );
```

### install nginx and set config files

```
[…]# vim /etc/nginx/conf.d/wordpress.conf
server {
    listen 443 ssl;
    server_name example.de

    ssl_certificate fullchain.pem;
    ssl_certificate_key privkey.pem;

    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    root /usr/share/wordpress;

    index index.php index.html index.htm;

    location / {
        try_files $uri $uri/ /index.php?$args;
    }

    location ~ \.php$ {
        fastcgi_split_path_info ^(.+\.php)(/.+)$;
        fastcgi_index index.php;
        include fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        fastcgi_param PATH_INFO $fastcgi_path_info;
        fastcgi_pass unix:/run/php-fpm/www.sock;
    }

     location ~ /\.ht {
        deny all;
    }
}
```

### start nginx server

```
systemctl restart nginx
systemctl reload nginx
```

