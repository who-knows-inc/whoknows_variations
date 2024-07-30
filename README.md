# Flask Variations



## Get started

cd into the src folder where `docker-compose.yml` is located and run the following command:

```bash
$ docker-compose up --build
```

## URLs

Who knows app: http://localhost:8080/

Who knows metrics endpoint: http://localhost:8080/metrics

The Prometheus web client: http://localhost:9090/

The Grafana dashboard: http://localhost:3000/ (login with `admin`/`admin`.)

If you are following the tutorial with intent to implement it for your group, remember to change the login credentials to what is posted in Teams. 

## Steps

1. [The Flask Prometheus setup and docker-compose.yml](./tutorial/1._Flask_Prometheus_Setup.md)

2. [The Grafana setup](./tutorial/1._Grafana_Setup.md)


## Warning

Running Prometheus and Grafana on the same server as the app is not recommended. Why do you think that is?

Besides the answer to the question above, it will probably be difficult to achieve with the server sizes we use. 

## Further work

If you are interested you can go to the Prometheus dashboard (http://localhost:9090/) and try out the *Prometheus query language*.

Here are some links for inspiration:

https://promlabs.com/promql-cheat-sheet/

https://blog.ruanbekker.com/cheatsheets/prometheus/