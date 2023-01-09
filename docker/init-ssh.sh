#!/bin/bash

set -uex

SSHD_PORT=2222

pacman -S --noconfirm openssh lsof
groupadd ssh
usermod -aG ssh "${USERNAME}"

# Setup sshd
mkdir -p /var/run/sshd
sed -i 's/session\s*required\s*pam_loginuid\.so/session optional pam_loginuid.so/g' /etc/pam.d/sshd
sed -i 's/#*PermitRootLogin prohibit-password/PermitRootLogin yes/g' /etc/ssh/sshd_config
sed -i -E "s/#*\s*Port\s+.+/Port ${SSHD_PORT}/g" /etc/ssh/sshd_config
# Need to UsePAM so /etc/environment is processed
sed -i -E "s/#?\s*UsePAM\s+.+/UsePAM yes/g" /etc/ssh/sshd_config

tee -a /usr/local/share/ssh-init.sh > /dev/null \
<< 'EOF'
#!/usr/bin/env bash

set -e 

# ** Start SSH server **
/usr/bin/sshd -o ListenAddress=0.0.0.0 2>&1 | tee /tmp/sshd.log > /dev/null
set +e
exec "$@"
EOF
chmod +x /usr/local/share/ssh-init.sh

ssh-keygen -A
