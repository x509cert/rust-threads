# rust-threads

I was not sure if threads in Rust running on Windows are real Windows threads or green threads. 
They are Windows threads as noted below.

<img width="675" alt="image" src="https://github.com/x509cert/rust-threads/assets/1674552/66ce6a07-b65b-4be2-b7ff-274c27148730">

The thread ID returned by thread::current().id() is a Rust value, however, and not the Windows TID.
