***Unicode environment***
```sh
sudo locale-gen en_US.UTF-8
sudo update-locale LANG=en_US.UTF-8
```

***Passwordless sudo***
```sh
sudo_user=$USER
sudo_conf=/etc/sudoers.d/nopassword
echo "${sudo_user} ALL=(ALL) NOPASSWD:ALL" | sudo tee $sudo_conf
sudo chmod 440 $sudo_conf
```

***Alternative program***
```sh
# editor
sudo update-alternatives --config editor
```
