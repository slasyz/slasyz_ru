# slasyz_ru

Sources of https://slasyz.ru/ webpage.


## Usage

To start a production server run:

```shell script
make prod
```

To start a development server run:

```shell script
make dev
```


## Architecture

Project consists of four containers:
- `database` container with PostgreSQL
- `backend` container
  - development version runs gunicorn with auto-reloading
  - development version runs gunicorn without auto-reloading
- `frontend` container
  - development version runs webpack dev server featuring hot reloading
  - production version runs webpack build with nginx serving its output
- `gateway` container that redirects external requests to inner containers


## Known issues

- webpack hot reloading works only with polling when project running in WSL outside WSL directories (e.g. in `/mnt/c/Users/...`)
- graceful shutdown of `backend` and `frontend` containers doesn't work correctly

## License
[MIT](https://choosealicense.com/licenses/mit/)
