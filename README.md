# RocketRange
Rocket (web framework) extension to provide video / file "range" support.

The purpose of this file is to provide a top level solution to Rocket not supporting the "Range" command for files. This allows us to create a custom response for the routing system.

Inorder to use this file, just download the file and use it within the source of your project. As this "project" matures it will eventually be put on crate.io but as of right know it's a nieche implementation that hasn't had much work.

As the project stands right now: 

 - Safari can play a supplied video
 - Network calls are missed (more than likely because of the bad file implmentation)
 - Chrome / Firefox will crash their Rocket thread.


Pull requests are more than welcomed. 
