package main

import (
	"fmt"
	"os"
)

func main() {
	name := os.Args[1]
	//get arguments from os.Args

	//get current path running the program

	err := os.Mkdir(name, os.ModePerm)
	if err != nil {
		//panic error
		fmt.Println(1, err)
		os.Exit(1)
	}

	var dir = "./" + name

	//create another directory in dir
	err = os.Mkdir(fmt.Sprintf("./%s/src", dir), os.ModePerm)
	if err != nil {
		//panic error
		fmt.Println(2, err)
		os.Exit(1)
	}

	//create a file in src
	f, err := os.Create(fmt.Sprintf("./%s/src/main.velt", dir))
	if err != nil {
		fmt.Println(3, err)
		os.Exit(1)
	}

	//close file
	err = f.Close()
	if err != nil {
		fmt.Println(4, err)
		os.Exit(1)
	}

	//create a dir in dir named build
	err = os.Mkdir(fmt.Sprintf("./%s/build", dir), os.ModePerm)
	if err != nil {
		fmt.Println(5, err)
		os.Exit(1)
	}

	//create Cargo.toml on build

	err = os.WriteFile(fmt.Sprintf("./%s/build/Cargo.toml", dir),
		[]byte(fmt.Sprintf(`
		[package]
name = '%s'
version = '0.1.0'
edition = '2021'
	[build]
	default-target = "x86_64-unknown-linux-gnu"
	`, name),
		),
		os.ModePerm)
	if err != nil {
		fmt.Println(6, err)
		os.Exit(1)
	}

	//create src on build
	err = os.Mkdir(fmt.Sprintf("./%s/build/src", dir), os.ModePerm)
	if err != nil {
		fmt.Println(7, err)
		os.Exit(1)
	}

	//create main.rs on src directory
	f, err = os.Create(fmt.Sprintf("./%s/build/src/main.rs", dir))
	if err != nil {
		fmt.Println(8, err)
		os.Exit(1)
	}

	f.Close()
}
