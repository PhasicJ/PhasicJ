# Use Ansible to Configure a macOS Host as a PhasicJ CI Server

1. Provision a macOS v10.15 ("Catalina") CI host. (Automation of this
   process is out of scope of this project.) You should manually:

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

4. Create a `hosts` file. See `hosts.SAMPLE`.

5. Run the playbook from this directory:

   ```sh
   ansible-playbook \
       --inventory-file hosts \
       --ask-become-pass \
       playbooks/install_phasicj_macos_ci_deps.yml
   ```
