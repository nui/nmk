package nmk

import (
	"encoding/json"
	"github.com/sirupsen/logrus"
	"io/ioutil"
	"path"
)

type NmkConfig struct {
	TmuxSettingEnvs []string
}

func ReadConfig(nmkDir string) *NmkConfig {
	config := path.Join(nmkDir, "config.json")
	var dto NmkConfig
	bytes, err := ioutil.ReadFile(config)
	if err != nil {
		logrus.Fatal(err)
	}
	if err := json.Unmarshal(bytes, &dto); err != nil {
		logrus.Fatal(err)
	}
	return &dto
}
