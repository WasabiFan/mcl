# Particle filter sandbox

This is a toy 2D Monte Carlo Localization implementation that I created for my own experience. It's not useful in any
real system and makes a variety of simplifying assumptions. Currently, proper laserscans are approximated using only two
distance measurements, "left" and "down", in a rectangular environment. It renders the set of particles graphically
along with the "true" pose.

This was intended to directly replicate the original "Monte Carlo Localization for Mobile Robots" by Dellaert et al.:
https://www.ri.cmu.edu/publications/monte-carlo-localization-for-mobile-robots/

<img src="https://user-images.githubusercontent.com/3310349/83313747-559d6480-a1cc-11ea-9551-f756ba99f498.png" alt="visualization screenshot" width="500"/>