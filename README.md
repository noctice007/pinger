Pings list of domains

## Usage:

```
echo "www.google.com" >> domains
echo "nonexisting.domain.xyz" >> domains
echo "www.facebook.com" >> domains
cat domains | pinger 
```

Output:

	www.google.com
	www.facebook

## Install:

```
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
ninja -j8
sudo ninja install
```