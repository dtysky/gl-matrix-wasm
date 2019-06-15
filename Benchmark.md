# Benchmark(Matrix4, 2015 RMBP, Chrome)

multiply(WASM) x 16,854,499 ops/sec ±1.69% (57 runs sampled)  
multiply(JS) x 11,531,234 ops/sec ±1.68% (57 runs sampled)  

fromRotationTranslationScale(WASM) x 23,856,918 ops/sec ±1.92% (59 runs sampled)  
fromRotationTranslationScale(JS) x 19,409,172 ops/sec ±2.04% (57 runs sampled)  

create(WASM) x 777,045 ops/sec ±1.40% (42 runs sampled)  
create(JS) x 14,290,306 ops/sec ±1.97% (60 runs sampled)  

clone(WASM) x 639,096 ops/sec ±4.35% (44 runs sampled)  
clone(JS) x 10,756,018 ops/sec ±2.17% (58 runs sampled)  

copy(WASM) x 30,181,487 ops/sec ±1.95% (58 runs sampled)  
copy(JS) x 21,462,134 ops/sec ±1.81% (58 runs sampled)  

fromValues(WASM) x 535,998 ops/sec ±3.53% (37 runs sampled)  
fromValues(JS) x 12,594,008 ops/sec ±2.13% (60 runs sampled)  

set(WASM) x 14,491,076 ops/sec ±1.85% (59 runs sampled)  
set(JS) x 26,017,141 ops/sec ±1.84% (58 runs sampled)  

getElements(WASM) x 82,019,949 ops/sec ±5.11% (52 runs sampled)  
getElements(JS) x 78,669,371 ops/sec ±5.39% (49 runs sampled)  

add(WASM) x 23,637,588 ops/sec ±1.77% (58 runs sampled)  
add(JS) x 13,540,795 ops/sec ±1.55% (56 runs sampled)  

sub(WASM) x 24,021,824 ops/sec ±1.76% (58 runs sampled)  
sub(JS) x 14,201,044 ops/sec ±1.29% (59 runs sampled)  

identity(WASM) x 35,785,034 ops/sec ±2.42% (57 runs sampled)  
identity(JS) x 25,593,068 ops/sec ±2.03% (55 runs sampled)  

transpose(WASM) x 27,352,164 ops/sec ±2.03% (58 runs sampled)  
transpose(JS) x 20,822,856 ops/sec ±2.05% (57 runs sampled)  

invert(WASM) x 17,258,766 ops/sec ±1.57% (59 runs sampled)  
invert(JS) x 11,898,150 ops/sec ±1.42% (58 runs sampled)  

adjoint(WASM) x 18,084,176 ops/sec ±1.68% (60 runs sampled)  
adjoint(JS) x 11,614,897 ops/sec ±1.07% (59 runs sampled)  

determinant(WASM) x 28,644,773 ops/sec ±1.80% (58 runs sampled)  
determinant(JS) x 28,516,174 ops/sec ±2.55% (57 runs sampled)  

translate(WASM) x 22,924,839 ops/sec ±1.59% (60 runs sampled)  
translate(JS) x 15,140,558 ops/sec ±2.11% (55 runs sampled)  

scale(WASM) x 25,101,108 ops/sec ±2.20% (56 runs sampled)  
scale(JS) x 17,273,866 ops/sec ±1.68% (58 runs sampled)  

rotate(WASM) x 12,504,970 ops/sec ±1.23% (57 runs sampled)  
rotate(JS) x 8,130,842 ops/sec ±1.23% (59 runs sampled)  

fromRotationTranslationScaleOrigin(WASM) x 19,989,752 ops/sec ±1.85% (58 runs sampled)  
fromRotationTranslationScaleOrigin(JS) x 15,336,144 ops/sec ±1.21% (60 runs sampled)  

lookAt(WASM) x 23,311,849 ops/sec ±1.53% (59 runs sampled)  
lookAt(JS) x 17,898,447 ops/sec ±1.73% (58 runs sampled)  
