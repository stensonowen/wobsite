
cd wobsite/static/blog
pandoc -s -i post.md -o post.html
cat post_header post.html > post.post
./../../convert static/ --input static/blog --output static/
