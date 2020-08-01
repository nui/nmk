.PHONY: default install uninstall

UPDATER_SCRIPT_NAME     := google-ddns-updater.sh
UPDATER_SCRIPT_BIN      := /usr/local/bin/$(UPDATER_SCRIPT_NAME)
SYSTEMD_UNIT_NAME       := google-ddns-updater.service
SYSTEMD_UNIT_FILE       := /etc/systemd/system/$(SYSTEMD_UNIT_NAME)

default: install

install:
	install -T $(UPDATER_SCRIPT_NAME) $(UPDATER_SCRIPT_BIN)
	cp $(SYSTEMD_UNIT_NAME) $(SYSTEMD_UNIT_FILE)
	systemctl daemon-reload
	systemctl start $(SYSTEMD_UNIT_NAME)
	systemctl enable $(SYSTEMD_UNIT_NAME)
	echo run sudoedit $(SYSTEMD_UNIT_FILE) to set required environment

uninstall:
	install -T $(UPDATER_SCRIPT_NAME) $(UPDATER_SCRIPT_BIN)
	systemctl stop $(SYSTEMD_UNIT_NAME)
	systemctl disable $(SYSTEMD_UNIT_NAME)
	rm $(SYSTEMD_UNIT_FILE)
	rm $(UPDATER_SCRIPT_BIN)
