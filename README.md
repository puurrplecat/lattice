# Lattice
###### An implementation of a post-quantum public key cryptography strategy.
It utilises ideas from the closest vector problem.

<br>

How does LEARNING WITH ERRORS work. 

<br>

It is proven to be as hard as solving the closest vector problem.

<br>

Suppose I have a system of equations of 10 variables. The more equations the better (i'm guessing).

<br>

A private key is a particular solution to this system of linear equations under some modulus.

<br>

In fact, you can first generate the private key, then finalise the system of equations with a simple multiply to a matrix of coefficients.

<br>

Then you introduce some error (best if there is a even mix of positive and negative as we'll see later).

<br>

To encrypt a bit, the user takes the system of equations, sums a few of them and call it N. This means the errors are cumulatively summed. If the errors don't have a spread of positive or negative, it can exceed the modulus we are working under and it doesn't work. To be honest I am not 100% clear if that's the reason.

<br>

Then if we want to encrypt 1, we add half the modulus to what N equals. To encrypt 0, we do nothing

<br>

Now decryption, if given N, it is very hard to tell if this is 0 or 1. But with the private key, we substitute it into N to get what the equation should equal, and we can easily tell if half of the mod was added or not. It is extremely hard without the private key to find what it should be.

<br>

This is similar to the closest vector problem in that it is impossible to find the closest vector to the error filled vector (as this will be our private key). and without the private key, you cannot decrypt every bit sensibly to get information.