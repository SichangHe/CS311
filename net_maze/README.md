# NetMaze

## Installation

Install [Elixir](https://elixir-lang.org).

```shell
brew install elixir
```

By installing Elixir, you get `elixir`, `elixirc`, `iex`, and `mix`.

## Run

```shell
mix run
```

## Example success run

```shell
$ export MESSAGE=sh623
$ mix run
Compiling 3 files (.ex)
Generated net_maze app

07:55:01.014 [info] Sent `id sh623
` to 67.159.95.167:51300.

07:55:01.482 [info] Received `query 51315` from primary connection.

07:55:01.690 [info] Sent `id sh623
` to 67.159.95.167:51315.

07:55:02.151 [info] Received `query 51350`.

07:55:02.360 [info] Sent `id sh623
` to 67.159.95.167:51350.

07:55:02.910 [info] Received `query 51335`.

07:55:03.218 [info] Sent `id sh623
` to 67.159.95.167:51335.

07:55:03.676 [info] Received `query 51311`.

07:55:03.884 [info] Sent `id sh623
` to 67.159.95.167:51311.

07:55:04.345 [info] Received `query 51333`.

07:55:04.552 [info] Sent `id sh623
` to 67.159.95.167:51333.

07:55:05.060 [info] Received `query 51301`.

07:55:05.269 [info] Sent `id sh623
` to 67.159.95.167:51301.

07:55:05.734 [info] Received `query 51347`.

07:55:05.943 [info] Sent `id sh623
` to 67.159.95.167:51347.

07:55:06.499 [info] Received `query 51319`.

07:55:06.706 [info] Sent `id sh623
` to 67.159.95.167:51319.

07:55:07.184 [info] Received `query 51330`.

07:55:07.394 [info] Sent `id sh623
` to 67.159.95.167:51330.

07:55:07.857 [info] Received `query 51320`.

07:55:08.098 [info] Sent `id sh623
` to 67.159.95.167:51320.

07:55:08.645 [info] Received `query 51302`.

07:55:08.953 [info] Sent `id sh623
` to 67.159.95.167:51302.

07:55:09.413 [info] Received `query 51325`.

07:55:09.621 [info] Sent `id sh623
` to 67.159.95.167:51325.

07:55:10.082 [info] Received `query 51342`.

07:55:10.291 [info] Sent `id sh623
` to 67.159.95.167:51342.

07:55:10.796 [info] Received `query 51322`.

07:55:11.004 [info] Sent `id sh623
` to 67.159.95.167:51322.

07:55:11.469 [info] Received `query 51317`.

07:55:11.676 [info] Sent `id sh623
` to 67.159.95.167:51317.

07:55:12.230 [info] Received `query 51331`.

07:55:12.535 [info] Sent `id sh623
` to 67.159.95.167:51331.

07:55:13.049 [info] Received `query 51328`.

07:55:13.257 [info] Sent `id sh623
` to 67.159.95.167:51328.

07:55:13.717 [info] Received `query 51307`.

07:55:13.925 [info] Sent `id sh623
` to 67.159.95.167:51307.

07:55:14.482 [info] Received `query 51312`.

07:55:14.789 [info] Sent `id sh623
` to 67.159.95.167:51312.

07:55:15.249 [info] Received `query 51334`.

07:55:15.514 [info] Sent `id sh623
` to 67.159.95.167:51334.

07:55:15.976 [info] Received `query 51319`.

07:55:15.976 [info] Resent `id sh623
` to 67.159.95.167:51319.

07:55:17.862 [info] Received `query 51337`.

07:55:18.169 [info] Sent `id sh623
` to 67.159.95.167:51337.

07:55:18.634 [info] Received `query 51306`.

07:55:18.842 [info] Sent `id sh623
` to 67.159.95.167:51306.

07:55:19.301 [info] Received `query 51323`.

07:55:19.510 [info] Sent `id sh623
` to 67.159.95.167:51323.

07:55:20.012 [info] Received `query 51341`.

07:55:20.224 [info] Sent `id sh623
` to 67.159.95.167:51341.

07:55:20.731 [info] Received `query 51331`.

07:55:20.731 [info] Resent `id sh623
` to 67.159.95.167:51331.

07:55:23.084 [info] Received `listen 51418`.

07:55:23.084 [info] Listening at port 51418 as requested.

07:55:25.439 [info] Port 51418 received connection.

07:55:27.606 [info] Received `query 51318`.

07:55:27.814 [info] Sent `id sh623
` to 67.159.95.167:51318.

07:55:28.307 [info] Received `query 51302`.

07:55:28.307 [info] Resent `id sh623
` to 67.159.95.167:51302.

07:55:29.945 [info] Received `query 51315`.

07:55:29.945 [info] Resent `id sh623
` to 67.159.95.167:51315.

07:55:30.545 [info] Received `listen 51444`.

07:55:30.545 [info] Listening at port 51444 as requested.

07:55:32.756 [info] Port 51444 received connection.

07:55:32.756 [info] Received `query 51330`.

07:55:32.756 [info] Resent `id sh623
` to 67.159.95.167:51330.

07:55:34.757 [info] Received `listen 51437`.

07:55:34.758 [info] Listening at port 51437 as requested.

07:55:36.940 [info] Port 51437 received connection.

07:55:36.941 [info] Received `query 51336`.

07:55:37.149 [info] Sent `id sh623
` to 67.159.95.167:51336.

07:55:37.281 [info] Received `query 51331`.

07:55:37.282 [info] Resent `id sh623
` to 67.159.95.167:51331.

07:55:37.632 [info] Received `query 51317`.

07:55:37.632 [info] Resent `id sh623
` to 67.159.95.167:51317.

07:55:38.445 [info] Received `listen 51447`.

07:55:38.446 [info] Listening at port 51447 as requested.

07:55:40.576 [info] Port 51447 received connection.

07:55:40.576 [info] Received `listen 51436`.

07:55:40.576 [info] Listening at port 51436 as requested.

07:55:41.317 [info] Port 51436 received connection.

07:55:41.317 [info] Received `query 51309`.

07:55:41.530 [info] Sent `id sh623
` to 67.159.95.167:51309.

07:55:41.530 [info] Received `query 51346`.

07:55:41.737 [info] Sent `id sh623
` to 67.159.95.167:51346.

07:55:42.028 [info] Received `query 51316`.

07:55:42.235 [info] Sent `id sh623
` to 67.159.95.167:51316.

07:55:42.235 [info] Received `query 51338`.

07:55:42.445 [info] Sent `id sh623
` to 67.159.95.167:51338.

07:55:42.695 [info] Received `query 51306`.

07:55:42.695 [info] Resent `id sh623
` to 67.159.95.167:51306.

07:55:42.833 [info] Received `query 51342`.

07:55:42.833 [info] Resent `id sh623
` to 67.159.95.167:51342.

07:55:42.906 [info] Received `query 51303`.

07:55:43.114 [info] Sent `id sh623
` to 67.159.95.167:51303.

07:55:43.667 [info] Received `query 51308`.

07:55:43.875 [info] Sent `id sh623
` to 67.159.95.167:51308.

07:55:43.875 [info] Received `query 51311`.

07:55:43.875 [info] Resent `id sh623
` to 67.159.95.167:51311.

07:55:44.061 [info] Received `query 51339`.

07:55:44.268 [info] Sent `id sh623
` to 67.159.95.167:51339.

07:55:44.341 [info] Received `query 51318`.

07:55:44.342 [info] Resent `id sh623
` to 67.159.95.167:51318.

07:55:44.631 [info] Received `query 51302`.

07:55:44.631 [info] Resent `id sh623
` to 67.159.95.167:51302.

07:55:44.731 [info] Received `query 51320`.

07:55:44.731 [info] Resent `id sh623
` to 67.159.95.167:51320.

07:55:45.092 [info] Received `query 51334`.

07:55:45.092 [info] Resent `id sh623
` to 67.159.95.167:51334.

07:55:45.232 [info] Received `query 51312`.

07:55:45.232 [info] Resent `id sh623
` to 67.159.95.167:51312.

07:55:45.477 [info] Received `query 51348`.

07:55:45.686 [info] Sent `id sh623
` to 67.159.95.167:51348.

07:55:45.825 [info] Received `query 51335`.

07:55:45.825 [info] Resent `id sh623
` to 67.159.95.167:51335.

07:55:46.009 [info] Received `query 51347`.

07:55:46.009 [info] Resent `id sh623
` to 67.159.95.167:51347.

07:55:46.146 [info] Received `query 51348`.

07:55:46.147 [info] Resent `id sh623
` to 67.159.95.167:51348.

07:55:46.741 [info] Received `query 51334`.

07:55:46.741 [info] Resent `id sh623
` to 67.159.95.167:51334.

07:55:46.756 [info] Received `query 51322`.

07:55:46.756 [info] Resent `id sh623
` to 67.159.95.167:51322.

07:55:47.557 [info] Received `query 51318`.

07:55:47.558 [info] Resent `id sh623
` to 67.159.95.167:51318.

07:55:47.558 [info] Received `query 51304`.

07:55:47.765 [info] Sent `id sh623
` to 67.159.95.167:51304.

07:55:47.765 [info] Received `query 51309`.

07:55:47.766 [info] Resent `id sh623
` to 67.159.95.167:51309.

07:55:47.893 [info] Received `query 51318`.

07:55:47.893 [info] Resent `id sh623
` to 67.159.95.167:51318.

07:55:47.962 [info] Received `query 51342`.

07:55:47.962 [info] Resent `id sh623
` to 67.159.95.167:51342.

07:55:48.226 [info] Received `query 51317`.

07:55:48.226 [info] Resent `id sh623
` to 67.159.95.167:51317.

07:55:48.337 [info] Received `query 51335`.

07:55:48.337 [info] Resent `id sh623
` to 67.159.95.167:51335.

07:55:48.564 [info] Received `query 51307`.

07:55:48.564 [info] Resent `id sh623
` to 67.159.95.167:51307.

07:55:48.656 [info] Received `query 51318`.

07:55:48.656 [info] Resent `id sh623
` to 67.159.95.167:51318.

07:55:49.095 [info] Received `query 51343`.

07:55:49.303 [info] Sent `id sh623
` to 67.159.95.167:51343.

07:55:49.419 [info] Received `query 51327`.

07:55:49.628 [info] Sent `id sh623
` to 67.159.95.167:51327.

07:55:49.768 [info] Received `query 51311`.

07:55:49.768 [info] Resent `id sh623
` to 67.159.95.167:51311.

07:55:49.963 [info] Received `query 51306`.

07:55:49.964 [info] Resent `id sh623
` to 67.159.95.167:51306.

07:55:50.094 [info] Received `query 51316`.

07:55:50.095 [info] Resent `id sh623
` to 67.159.95.167:51316.

07:55:50.106 [info] Received `query 51320`.

07:55:50.106 [info] Resent `id sh623
` to 67.159.95.167:51320.

07:55:50.250 [info] Received `query 51330`.

07:55:50.250 [info] Resent `id sh623
` to 67.159.95.167:51330.

07:55:50.737 [info] Received `query 51340`.

07:55:51.052 [info] Sent `id sh623
` to 67.159.95.167:51340.

07:55:51.504 [info] Received `query 51335`.

07:55:51.505 [info] Resent `id sh623
` to 67.159.95.167:51335.

07:55:51.513 [info] Received `query 51315`.

07:55:51.513 [info] Resent `id sh623
` to 67.159.95.167:51315.

07:55:51.561 [info] Received `query 51332`.

07:55:51.768 [info] Sent `id sh623
` to 67.159.95.167:51332.

07:55:52.082 [info] Received `query 51348`.

07:55:52.082 [info] Resent `id sh623
` to 67.159.95.167:51348.

07:55:53.723 [info] Received `status success` from primary connection.

07:55:53.723 [warning] NetMaze.GenServer stopping at `success`.
```
