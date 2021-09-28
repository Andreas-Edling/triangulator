# Triangulator

![ci](https://github.com/Andreas-Edling/triangulator/actions/workflows/rust.yml/badge.svg)

Triangulator is a library written in [Rust](https://www.rust-lang.org/) that given a set of 2D points, it creates a [Delaunay triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation). 
I.e. it minimizes acuteness of triangles. 
There are two ways to use the library, either call fn triangulate(..) which will give the final triangulation:
```
   use triangulator::*;
   let points = [Point::new(0.,0.), Point::new(1.,0.), Point::new(1.,1.), Point::new(0.,1.), Point::new(0.5,0.5)];
   let triangles = triangulate(&points).unwrap();
```

or, for getting transitional triangulations:
```
   use triangulator::*;
   let points = [Point::new(0.,0.), Point::new(1.,0.), Point::new(1.,1.), Point::new(0.,1.), Point::new(0.5,0.5)];

   let mut triangulator = Triangulator::new();
   triangulator.initial_triangulation(&points).unwrap();
   while triangulator.do_step(&points) {
       let triangles = triangulator.get_triangles();
       // do something with triangles
   }
```



Sample of what a triangulation will look like:

![triangulation](https://user-images.githubusercontent.com/11133044/135079563-5bdc6adb-3aae-4e58-aadb-fe833472c39b.gif)
