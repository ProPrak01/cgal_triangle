#include <CGAL/Polygon_2.h>
#include <CGAL/Exact_predicates_inexact_constructions_kernel.h>
#include "triangle.h"

typedef CGAL::Exact_predicates_inexact_constructions_kernel Kernel;
typedef CGAL::Polygon_2<Kernel> Polygon_2;
typedef Kernel::Point_2 Point_2;

double compute_area(double x1, double y1, double x2, double y2, double x3, double y3) {
    Polygon_2 p;
    p.push_back(Point_2(x1, y1));
    p.push_back(Point_2(x2, y2));
    p.push_back(Point_2(x3, y3));
    return p.area();
}

void compute_centroid(double x1, double y1, double x2, double y2, double x3, double y3, float* cx, float* cy) {
    float ccx = (x1 + x2 + x3) / 3.0;
    float ccy = (y1 + y2 + y3) / 3.0;
    
    *cx = ccx;
    *cy = ccy;
}