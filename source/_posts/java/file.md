---
title: file 文件流
categories: Java
---

## 基本文件流的使用

### 字符流：

```java
package file;

import org.junit.Test;

import java.io.File;
import java.io.FileReader;
import java.io.IOException;

/**
 * 基本文件流操作
 */
public class BaseFile {
    public static void main(String[] args) {
        BaseFile baseFile = new BaseFile();
        baseFile.readFile();
    }

    /**
     * file 基本使用
     */
    public void baseFile() {
        File file = new File("hello.txt");
        System.out.println(file.getAbsoluteFile());
        // /Users/heart/Desktop/java/stu/hello.txt 相较于当前工程
    }

    /**
     * 文件数据读取
     */
    public void readFile() {
        FileReader fr = null;
        try {
            fr = new FileReader("src/file/base-file.txt");
            while (true) {
                int a = fr.read();
                if (a == -1) break;
                // 表现形式为 uft8 可以使用char转为字符串
                // 换行符的 操作为 10
                System.out.println(a);
            }

        } catch (Error | IOException e) {
            // 文件如果不存在 则会报错 FileNotFoundException
            e.printStackTrace();
        } finally {
            try {
                // 关闭流的读取操作
                // JVM无法对自动对I/O流进行关闭 因此 需要我们自己去关闭 否则会造成内存泄漏
                // 保证流的能够正常的关闭
                if (fr != null) {
                    fr.close();
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }


    @Test
    public void test() {
        File file = new File("hello.txt");
        System.out.println(file.getAbsoluteFile());
        // 相较于当前的Module
        // /Users/heart/Desktop/java/stu/hello.txt
    }

}

```

> char[] 读取文件

```java
@Test
public void test(){
	char[]cr=new char[5];
	FileReader fr=null;
	try{
		fr=new FileReader("src/file/base-file.txt");
		int len=0;
		// 实例化一个 FileReader类 设置默认的文件路径 如果read(char[] cbuf)传递了一个char类型的参数 则会将变量读取到参数中
		while((len=fr.read(cr))!=-1){
			for (int i=0;i<cr.length;i++){
				System.out.print(cr[i]);
			}
			System.out.println();
		}
	}
	catch(Error|FileNotFoundException e){
		e.printStackTrace();
	}
	catch(IOException e){
		throw new RuntimeException(e);
	}
	finally{
		try{
			if(fr!=null){
				fr.close();
			}
		}
		catch(IOException e){
			e.printStackTrace();
		}
	}
}
```

```java
@Test
public void readTest(){
	FileReader fr=null;
	try{
		File file=new File("src/file/base-file.txt");
		char[]cr=new char[5];
		System.out.println(file.getAbsoluteFile());
		fr=new FileReader(file);
		int len=0;
		while((len=fr.read())!=-1){
			System.out.print((char)len);
		}
	}
	catch(Error|IOException e){
		e.printStackTrace();
	}
	finally{
		try{
			if(fr!=null){
				fr.close();
			}
		}
		catch(IOException e){
			e.printStackTrace();
		}
	}
}
```

> 文件写入类
> 文件可以不存在
> 如果文件不存在 则会创建文件 在写入内容
> 如果文件存在：

```java
@Test
public void readTest(){
	FileReader fr=null;
	try{
		File file=new File("src/file/base-file.txt");
		char[]cr=new char[5];
		System.out.println(file.getAbsoluteFile());
		fr=new FileReader(file);
		int len=0;
		while((len=fr.read())!=-1){
			System.out.print((char)len);
		}
	}
	catch(Error|IOException e){
		e.printStackTrace();
	}
	finally{
		try{
			if(fr!=null){
				fr.close();
			}
		}
		catch(IOException e){
			e.printStackTrace();
		}
	}
}
```

```java
/**
 * 将 hello.txt  文件拷贝到 hello.txt 中
 */
@Test
public void copyFile(){
	FileReader fr=null;
	FileWriter fw=null;
	try{
		fr=new FileReader("src/file/hello.txt");
		fw=new FileWriter("src/file/hello1.txt");
		int len=0;
		while((len=fr.read())!=-1){
			fw.write((char)len);
		}
		// 强制将缓存中的数据写入流中 调用fw.close() 也会调用这个方法
		// fw.flush();
	}
	catch(Error|FileNotFoundException e){
		e.printStackTrace();
	}
	catch(IOException e){
		throw new RuntimeException(e);
	}
	finally{
		try{
			if(fr!=null)fr.close();
			if(fw!=null)fw.close();
		}
		catch(IOException e){
			e.printStackTrace();
		}
	}
}
```

### 字节流

```java
/**
 * 使用字节流去拷贝图片
 */
@Test
public void copyImage(){
	FileInputStream fr=null;
	FileOutputStream fw=null;
	try{
		// 读取操作
		fr=new FileInputStream("src/file/image.png");
		// 写入操作
		fw=new FileOutputStream("src/file/image1.png");
		int len=0;
		// read -1 ~ 255 字节流
		while((len=fr.read())!=-1){
			System.out.println(len);
			fw.write(len);
		}
	}
	catch(FileNotFoundException|Error e){
		e.printStackTrace();
	}
	catch(IOException e){
		throw new RuntimeException(e);
	}
	finally{
		try{
			if(fr!=null)fr.close();
			if(fw!=null)fw.close();
		}
		catch(IOException e){
			e.printStackTrace();
		}
	}
}
```

字节流 如果读取字符的话 英文和数字不会乱码 中文可能会乱码

> 1 个中文占用 3 个字节
> .jpg .mp3 .mp4 .avi .doc .ppt 等都可以使用字节流处理

### 缓冲流

- BufferedInputStream
- BufferedOutputStream
- BufferedWriter
- BufferedReader

> 关闭外层的流 也会将内层的流一起关闭

```java
/**
 * 缓冲流
 * 字符流处理
 */
@Test
public void bufferFileClass()throws FileNotFoundException{
	BufferedReader reader=null;
	BufferedWriter writer=null;
	try{
		File originFile=new File("src/file/hello.txt");
		FileReader fileReader=new FileReader(originFile);
		reader=new BufferedReader(fileReader);
		File remoteFile=new File("src/file/hello1.txt");
		FileWriter fileWriter=new FileWriter(remoteFile);
		writer=new BufferedWriter(fileWriter);
		int len=0;
		while((len=reader.read())!=-1){
			writer.write(len);
		}
	}
	catch(Error|FileNotFoundException e){
		e.printStackTrace();
	}
	catch(IOException e){
		throw new RuntimeException(e);
	}
	finally{
		try{
			if(reader!=null)reader.close();
			if(writer!=null)writer.close();
		}
		catch(IOException e){
			throw new RuntimeException(e);
		}
	}
}
```

```java
/**
 * 缓冲流
 * 字节流
 */
@Test
public void bufferFileClass()throws FileNotFoundException{
	BufferedReader reader=null;
	BufferedWriter writer=null;
	try{
		File originFile=new File("src/file/hello.txt");
		FileReader fileReader=new FileReader(originFile);
		reader=new BufferedReader(fileReader);
		File remoteFile=new File("src/file/hello1.txt");
		FileWriter fileWriter=new FileWriter(remoteFile);
		writer=new BufferedWriter(fileWriter);
		int len=0;
		while((len=reader.read())!=-1){
			writer.write(len);
		}
	}
	catch(Error|FileNotFoundException e){
		e.printStackTrace();
	}
	catch(IOException e){
		throw new RuntimeException(e);
	}
	finally{
		try{
			if(reader!=null)reader.close();
			if(writer!=null)writer.close();
		}
		catch(IOException e){
			throw new RuntimeException(e);
		}
	}
}
```

BufferedInputStream 内部提供了缓冲区

```java
private static int DEFAULT_BUFFER_SIZE=8192; // 提供了8kb的缓冲区
```

手动调用 flush 将缓冲区的内容写入到文件中（并且清空缓冲区）

```java
FileWriter fw=null;
File file=new File("src/file/hello.txt");
// 不再原有的文件进行添加 false 则覆盖原来的文件 true 则咋原来的文件下追加
fw=new FileWriter(file,true);
fw.write("hello read");
// 将缓存流写入到文件中
fw.flush();
```

`readLine` 每次读取一行的数据

```java
File originFile=new File("src/file/hello.txt");
FileReader fileReader=new FileReader(originFile);
reader=new BufferedReader(fileReader);

File remoteFile=new File("src/file/hello1.txt");
FileWriter fileWriter=new FileWriter(remoteFile);
writer=new BufferedWriter(fileWriter);

String str="";
while((str=reader.readLine())!=null){
	// str中不会包括换行符
	writer.writer(str);
	// 可以使用newLine()换行
	writer.newLine();
	
}
```

### 图片加解密
```java
@Test
    public void secretTest() {
	BufferedInputStream bis = null;
	BufferedOutputStream bos = null;
	try {
		bis = new BufferedInputStream(new FileInputStream("src/file/image.png"));
		bos = new BufferedOutputStream(new FileOutputStream("src/file/image-secret.png"));
		int len = 0;
		while ((len = bis.read()) != -1) {
			// 加密操作
			bos.write(len ^ 5);
		}
	}
	catch (Error | IOException e) {
		e.printStackTrace();
	}
	finally {
		try {
			if (bis != null) bis.close();
			if (bos != null) bos.close();
		}
		catch (IOException e) {
			e.printStackTrace();
		}
	}
}
```
解密：
```java
@Test
    public void decode() {
	BufferedInputStream bis = null;
	BufferedOutputStream bos = null;
	try {
		bis = new BufferedInputStream(new FileInputStream("src/file/image-secret.png"));
		bos = new BufferedOutputStream(new FileOutputStream("src/file/image-decode.png"));
		int len = 0;
		while ((len = bis.read()) != -1) {
			// 解密操作
			bos.write(len ^ 5);
		}
	}
	catch (Error | IOException e) {
		e.printStackTrace();
	}
	finally {
		try {
			if (bis != null) bis.close();
			if (bos != null) bos.close();
		}
		catch (IOException e) {
			e.printStackTrace();
		}
	}
}
```