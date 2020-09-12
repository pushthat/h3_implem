# Goal

## Exo

un service GRPC (tonic rust)

qui power des polygons

que tu puisse query avec des lat/lng

ou h3 id, que tu puisse insert des polygons

https://github.com/uber/h3
https://eng.uber.com/h3/

## Bonus

Une API graphql (https://github.com/async-graphql/async-graphql)

## Status

- [x] indexing system (h3)
- [x] query system
- [x] insert system
- [x] grpc api
- [ ] graphql

## Infos

You can modify resolution layer in the ResolutionDetails enum

./test-grpc.sh contain example call

## example :

This is the best reprentation I could make but this does not really reflect the polygon stored has there's a lot of line between each

Zone to cover :

![sample-1](./media/sample-1.png)

first level of resolution :

![sample-2](./media/sample-2.png)

second level of resolution without what is already covered:

![sample-3](./media/sample-3.png)

last level of resolution without what is already covered:

![sample-4](./media/sample-4.png)

This way the map is covered
