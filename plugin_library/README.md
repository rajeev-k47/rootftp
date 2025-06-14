### 1. **Image Compressor**: (libimage_compressor_plugin.so)
   -  Can be used to compress images.
   -  No use of optional input file.
   -  Extensions: ['jpeg','jpg','png']

### 2. **OCR Plugin** : (libocr_plugin.so)
   - Can be used to extract text from images.
   - Dependencies: leptonica-devel tesseract-devel clang tesseract-langpack-eng
     
     ```
     sudo dnf install leptonica-devel tesseract-devel clang tesseract-langpack-eng  //For RPM based distros
     ```
   -  Extensions: ['jpeg','jpg','png']

### 3. **CPP Comipler**: (cpp_compiler_plugin.so)
   -  Can be used to compile and run cpp files.
   -  Optional input file for stdin (Naming-> input.in).
   -  Extensions: ['cpp']
