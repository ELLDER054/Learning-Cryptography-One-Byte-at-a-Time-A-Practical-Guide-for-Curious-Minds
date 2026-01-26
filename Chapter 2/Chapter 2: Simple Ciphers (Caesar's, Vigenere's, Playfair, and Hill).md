# Simple Ciphers
To begin, it's usually good to start with simpler but weaker ciphers. The benefit of this starting out is that it will help solidify the vocabulary while getting you into the fun of encrypting, decrypting, and even attacking secret messages. In cryptography, there are generally two states that a message can be in: `plaintext` and `ciphertext`.

Plaintext
: data that has not been encrypted and is in its plain form

Ciphertext
: data that *has* been encrypted and therefore should not make sense to anyone other than the intended reader(s)

For the sake of this introduction, messages in `plaintext` will be lower-case while messages in `ciphertext` will be entirely upper-case.
It is helpful to remember this information, as this notation is common even in higher level textbooks.

`hello -> MJQQT`

`cryptography -> MBIZDYQBKZRI`

If you had very little background in playing with small ciphers like this and I asked you to decrypt the ciphertext `wxsstcvtb`, you may look at me in surprise. Though that may seem like a jump in difficulty, those of you that are familiar with the **Caesar's Cipher** would welcome the challenge. And hopefully, everyone reading this through to the end will have the tools and knowledge necessary to tackle that head on.

# What is the Caesar's Cipher?
The Caesar's Cipher is a simple `substition cipher` that was reportedly used by Julius Caesar during the height of his military career. Again, while it is not considered a secure encryption method, it does provide for a beginner-friendly way to dip your toes into the world of cryptography.

Substitution Cipher
: a method of encryption where each character of the plaintext is replaced by another unique character

So the way a Caesar's Cipher works is that every letter is shifted by a certain amount of letters to a new letter. Those new letters then make up the ciphertext of the message. It relies on the false assumption that attackers can't tell what shift was used. There are really only 25 possible shifts that a Caesar's Cipher can make use of, making it so that if it is *verified* that a Caesar's Cipher is being used, it is extremely trivial to break.

To understand better what a shift really means, it can be helpful to look at a few examples.
