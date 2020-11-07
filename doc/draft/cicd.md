What i want :
on each commit on master :
    build & test
    build & push "latest" docker image

on each commit on "release_{version}" :
    build & push "{version}" docker image
    maintain a github release "{version}" with linux & windows binary