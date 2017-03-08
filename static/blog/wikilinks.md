
# Summary


# Original Approach

Early on, the process of loading links and seeking paths was quite straightforward. It was divided into three real components. 

1. A python script would parse the wikipedia dump's text file, looking for the pattern that would indicate an internal link, (e.g. "... page content with a [[link article title|link friendly name]]..."). The script would produce a text file associating each article title with the titles of all of its children. This was useful because parsing was somewhat expensive; I/O is a bottleneck, so cutting out most of the text that would be ignored anyway is helpful. This process isn't ideally efficient, taking about half an hour on one core for the full English dump, but it only needs to be run once per dump. 

2. The next step was table generation. To make lookups fast, everything is stored in a pretty minimalistic hash table, mapping a hash of an article title to a struct containing its title and children. So for each parent/child title pair in the python parse data, each string was hashed and looked up (and inserted if necessary) to get its associated index. Then the collection of children at `table[parent_index]->children` was updated to include the child index. This way each string only needs to be stored once, and all of the links can be represented as 32-bit integers, which are small and cheap to operate on.

3. Performing a breadth-first search from one index to another is just a matter of expanding the pool of reachable indices until it includes the destination index. If `dst_index` is a child of `src_index`, then the search is over; otherwise each of `src_index`'s children is added to a pool of seen indices. For each element in this pool (which is currently a child of `src_index`), if `dst_index` is among its children, then the search is over (there might be other equally long searches, but there can't be any shorter ones); otherwise all of its children (which are `src_id`'s grandchildren) that have not been seen before are added to a new pool of descendants to be searched, repeating until the `dst_index` is found or a max iteration is reached. This effectively populated sets of indices that are reachable in no more than `n` iterations for increasing values of `n`. It is as efficient as expected, only storing what it needs (a set of the `n`th generation of descendants and a set of all other reachable indices) and not entering into any cycles. This doesn't scale particularly well: articles can have several hundred children (the average is probably around ~80), so that means storing 80<sup>i</sup>; a search of depth 10 could require searching through <i>billions</i> of integers. Fortunately, most searches are much shorter than this, but it meant that searches could have wildly unpredictable run times, from milliseconds to minutes.

## Speed/Memory optimizations

There were a handful of ways I sped this process up over the course of a few months. It initially had almost comically bad resource consumption, requiring an ~80GB pagefile and ~13 hours to load into memory (from the end of the python parsing step to the beginning of the first search), but this improved drastically after heavy refactoring and profiling. 

* Refactoring was the first major step; it didn't speed things up per se, but it turned the codebase from a mess of delicate unreadable garbage into something much easier to hack on.
* Switching from using a linked list to store children indices to a vector. My initial thought process was that the hash table took up lots of contiguous memory so some non-contiguous memory would lead to better usage of RAM. In reality, it just turned every 32-bit integer into a list node that also contained two 64-bit pointers, and it killed any semblance of cache locality populating/searching could have used. I was fresh out of data structures; can you blame me?
* Reduced a layer or two of indirection in the entry struct and hash table. I hadn't given this much consideration while initially writing the program, but this helped to reduce the number of cache misses and unecessary pointer storage.
* Switched from running inside MS Visual Studio to compiling with gcc and running on linux. I didn't do this before because I didn't have a machine running linux with enough memory (and I thought swap files wouldn't handle the load as well as a pagefile had), but when I finally fixed it up to compile in gcc there was a large performance improvement. I figured running in Visual Studio probably hooked memory allocations (or maybe the windows allocator is less suited to this than libc's `malloc`?), but population times went from minutes to seconds.
* I parallelized the table population process. Because data loaded by the table was the output of my own python script, it was easy enough to split the data into multiple chunks that could all be read concurrently (more on this below).

Overall, this worked quite well. Some of these changes had drastic effects on memory consumption, which eliminated the need for a pagefile and increased the effects of caching, in turn drastically improving the population time: at the end of the day this step took about five minutes and under 16GB of RAM.

## Thread Safety

Parallelizing 

## Shortcomings


# Rust rewrite

## Why Rust

## Addressing Shortcomings

## State Machine Pattern

## Pageranks

## Future Speedups

* Reduce cache misses by inlining parents/children in ?Sized structs
* Use faster hashing via bitwise operations
* Rewrite perfect hashing crate



