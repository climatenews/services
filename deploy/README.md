# Use docker as non-root user
sudo groupadd docker
sudo usermod -aG docker ${USER}
su -s ${USER}