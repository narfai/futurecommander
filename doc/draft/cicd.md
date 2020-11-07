What i want :
on each commit on master :
    build & test
    build & push "latest" docker image
    maintain a github a "latest" release with linux & windows binary

on each commit on "release_{version}" :
    build & push "{version}" docker image
    maintain a github release "{version}" with linux & windows binary