ng build --prod && \
    scp dist/demo/* pi@young.local:~/tmp_http/ && \
    ssh pi@young.local "sudo mv ~/tmp_http/* /var/www/wavesuit && sudo systemctl restart apache2"
