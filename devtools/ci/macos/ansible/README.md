# Use Ansible to Configure a macOS Host as a PhasicJ CI Server

1. Provision a macOS v10.15 ("Catalina") CI host. Automation of this process
   is currently out of scope of this project. You should manually:

    - Create a CI host by either
        - Installing a macOS instance on Apple hardware, or
        - Installing macOS within a VM. (E.g., VMware Fusion and Parallels
          can run macOS VMs on Apple hardware).
        - Set up a user with admin privileges on the CI host.
        - Set up SSH login from the Ansible host to the CI host with this admin
          user.
    - Install homebrew on the CI host.
    - Create a "standard" (i.e. non-admin) user with the UNIX-name `phasicj-ci`.
    - Install XCode from the App Store. (The Command Line Tools are not
      sufficient.)

    Note that the Ansible host and the CI host can in principle be the same
    system.

2. Install Ansible locally.

3. Run `ansible-galaxy collection install --requirements-file requirements.yml`
   to ensure that all required Ansible collections are installed on the Ansible
   host. (E.g., we need `ansible.general` for Homebrew support).

4. Create an `inventory` file. See `inventory.SAMPLE`.

5. Create a `files/user/ssh/authorized_keys` file. These are the SSH keys which
   which will be able to login to the `phasicj-ci` user. See the
   `files/user/ssh/authorized_keys.SAMPLE` file.

6. Create a `files/user/aliases.msmtp` file. This is used to specify the email
   address where CI results are sent. See the `files/user/aliases.msmtp.SAMPLE`
   file.

7. Create a `secrets/user/msmtprc` file. This contains the email-sending
   credentials to be used. See the `secrets/user/msmtprc.SAMPLE` file.

8. Run the playbook from this directory:

   ```sh
   ansible-playbook \
       --ask-become-pass \
       install_dependencies.yml
   ```
