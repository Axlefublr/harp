Registers now hold an array rather than an object. \
Harp is made to be an easy interface for quite arbitrary editor (if even) actions, and so the 4 allowed fields were also arbitrary. Two of them being specifically numbers isn't great either. \
I'm basically assuming that all you'll ever want to save is some path, maybe line number and column number, and I give you this singular field called "extra" for everything else. \
I know from experience that what happens in practice, is that you will find yourself abusing that extra field to hold more data potentially, splitting it by some arbitrary string, or parsing it into what you actually want. \
This ends up being a mess that really doesn't need to exist, when in reality you just wanted more fields.

So why was it designed that way in the first place? To make serialization and deserialization straightforward and non-guessy. \
“At most, you'll have 4 fields” is a nice enough promise. \
But ultimately that's stupid and limiting design, when I realized that *just* using an array is simpler both on the library side, and on the *user*'s side. \
As in, on the side of the user of the cli harp, and on the side of the user of the library harp.

It's simply overall better.

In all of my usecases, you know how many values are set for a given register, as you know what the setter sets; It doesn't blammo an unknown amount of values; And if it does, that's probably what you want anyway.

I removed the clear action for now just so that I can push out this update sooner. I plan to reimplement it in the future, and also provide ways to *view* existing sections and existing registers in each sections, to allow for editors to show a preview. But that's for later.

I switched from yaml to json, because I realized that json has more available and known tooling; `yq` knowers are a subsection of `jq` knowers, basically. \
So your previous `harp.yml` is no longer touched, and a new `harp.jsonc` is used instead. If you want to transfer your data, convert the yaml into json and convert the objects that hold properties, into an array.
