# Assessment: Heatwave in The Netherlands


## How to setup the env?
1. Run docker-engine in you machine
1. run `docker compose build`
1. run `docker compose up -d`


It will set up and launch the following services

- spark-master, gui available on [localhost:7078](http://localhost:7078/) (no auth)
- spark-worker, gui available on [localhost:8082](http://localhost:8082/) (no auth) 
- jupyter-notebook, available on [localhost:8888](http://localhost:8888/) (token can be found in container's log) 


### 1BRC Results:

Setup
    - Master: 1x
    - Workers: 4x
    - Run time: ~3min