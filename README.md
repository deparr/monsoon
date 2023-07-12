# monsoon :cyclone: 
A cli weather fetcher

**:rotating_light: this is terribly inefficient and mostly intended for my personal use :rotating_light:**

---

A program that fetches the current weather using [OpenWeather API](https://openweathermap.org/api). I'm sure something like this already exists, I just wanted an excuse to write a (terrible) Rust program.

Right now it only has the very specific functionality of appending the latest data to a log file (which is read by a `listen` variable in my [eww](https://github.com/elkowar/eww) status bar).

### Configuration
monsoon expects a config file at either `$XDG_CONFIG_HOME/monsoon/config.json` or `$HOME/.config/monsoon/config.json`.

Currently, the config file is only used to store the parameters used in the API call.

An example config:
(all keys required)
```json
{
    "key": "your-api-key",
    "lat": "40.76",
    "lon": "-74.01",
    "units": "imperial"
}
```

Currently supported keys:

`key`, the api key string.

`lat`, the target location lattitude as a float

`lon`, the target location longitude as a float 

`units`, the unit of measurement returned by the api, can be `"metric"`, `"imperial"`, or `"default"` (Celsius, Fahrenheit, and Kelvin respectively).

See the [api docs](https://openweathermap.org/current) for more info.
