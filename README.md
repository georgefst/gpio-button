~~Runs a command when (pull-up) button on given GPIO port is pressed.~~

~~``` bash
./gpio-button 15 sudo poweroff
```

# EDIT
This is pretty redundant with the newer GPIO interface, which I'm moving my projects towards. The above would become
```bash
gpiomon -f -n 1 gpiochip0 15 && sudo poweroff
```
