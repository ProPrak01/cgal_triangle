#include <rust/cxx.h>
#ifndef TRIANGLE_H
#define TRIANGLE_H

extern "C" {
    double compute_area(double x1, double y1, double x2, double y2, double x3, double y3);
    void compute_centroid(double, double, double, double, double, double, float*, float*);
}
#endif