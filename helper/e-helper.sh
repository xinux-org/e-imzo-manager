_log() {
	local _type="1"      # 4, trace by default, 3 for warning
	local _kind="error:" # warning message by default

	# Check if arguments are supplied
	if [ $# -eq 0 ]; then
		echo "$(tput bold)$(tput setaf $_type)$_kind$(tput sgr0)$(tput bold) no type supplied to log function!$(tput sgr0)"
		return 1
	fi

	if [[ $1 == "warn" ]]; then
		_type="3"
		_kind="warning:"
	elif [[ $1 == "trace" ]]; then
		_type="4"
		_kind="trace:"
	fi

	# Check if the message ($2) is supplied
	if [ -z "$2" ]; then
		echo "$(tput bold)$(tput setaf $_type)$_kind$(tput sgr0)$(tput bold) no message supplied to log function!$(tput sgr0)"
		return 1
	fi

	echo "$(tput bold)$(tput setaf $_type)$_kind$(tput sgr0)$(tput bold) $2$(tput sgr0)"
}

echo

echo "$(tput bold)$(tput setaf 4)Welcome to Xinux'es E-IMZO bootstrapper!$(tput sgr0)"
echo "$(tput rev)$(tput setaf 4)Remember: this script will use sudo power to create certain folders/files at '/'!$(tput sgr0)"

echo

_log "trace" "Let's see if '/media' folder exists in your system..."
if [ -d "/media" ]; then
	_log "trace" "Good! The '/media' folder exists..."
else
	_log "warn" "Whoops, '/media' folder doesn't exist, I'm gonna create it!"
	_log "warn" "You may probably need to type password as whatever after '/' is system path."
	# sudo mkdir -p /media
fi

if [ -d "/media/DSKEYS" ]; then
	_log "trace" "Good! The folder exists. Hey, wait-a minute, this isn't your first time isn't it?)"
else
	echo
	echo "$(tput bold)Look, I need ask you something. Is your keys in your flash drive or you want to keep them here in your pc?$(tput sgr0)"
	read -p "$(tput bold)[Yy]es for device, [Nn]o for flash drive:$(tput sgr0) " -n 1 -r
	echo
	if [[ $REPLY =~ ^[Yy]$ ]]; then
		_log "trace" "Ok! I'll create /media/DSKEYS directory for you to store your own keys in your device!"
		# sudo mkdir -p /media/DSKEYS
	elif [[ $REPLY =~ ^[Nn]$ ]]; then
		_log "warn" "Ok! However, remember that, when you mount your flash drive, you should mount it to /media path"
		_log "warn" "and your flash drive should include 'DSKEYS' folder with keys inside it."
		_log "warn" "Example:"
		_log "warn" "   # If it's already mounted, unmount it first"
		_log "warn" "   sudo mount /dev/sdc /media"
		exit 0
	else
		_log "warn" "Ok, I kinda didn't understand it, but I'll count it as yes -> device!"
		# sudo mkdir -p /media/DSKEYS
	fi
fi

_log "trace" "Good! We create all necessary folders, now you need to copy paste your keys to designated path."
_log "trace" "I'll open the 'DSKEYS' folder for you in your default file manager in 3 seconds! "
sleep 3
xdg-open /media/DSKEYS
