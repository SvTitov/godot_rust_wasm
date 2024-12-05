# Export to Web
1. The name of RUST project and Godot must be different due to file overriting.
2. I use emcc 3.1.39
  ```sh
.\emsdk.bat install 3.1.39 
.\emsdk.bat activate 3.1.39 --permanent
```
3. Export config
   ![image](https://github.com/user-attachments/assets/53c4eb6d-e589-4937-981b-f3f841f7983f)

   ![image](https://github.com/user-attachments/assets/f1537005-ff1b-4cc5-87d6-c7c2f5c60ff5)


4. In export folder add gamedev.py. There's also must be index.html file (in my case I renamed {ProjectName} to index).
5. Open `localhost:8000`

