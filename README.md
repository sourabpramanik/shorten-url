This project is built using Vercel, Neon and Rust. It is a link managing application that has a CLI to create, read and delete short URLs. I have used Vercel to deploy a serverless function for redirection to long URLs, it is easy, fast and free.

## What is URL shortener and why do we need it?

URL shortener is a service that is used create short URLs that works as an alias for long URLs, this is important when you have a long URL (e.g. https://chart.apis.google.com/chart?chs=500x500&chma=0,0,100,100&cht=p&chco=FF0000%2CFFFF00%7CFF8000%2C00FF00%7C00FF00%2C0000FF&...) and sharing this with others is not that neat, what if you snap out some characters at the end of the URL. 

It becomes really hard this way, that is where we can use a URL shortening service that can generate an alias for the long URLs having comparitively very small length. The other benefits of having a short URLs or aliases are you can customise how you want your URL to appear(e.g. https://foo.baz/2fGH553), generate analytics like number of users clicked, country of origin, and many more. 

Many big companies like Twitter or X uses this service for sharing any content outside thier application. 

## How does it work?
![alt text](/assets/image.png)

User enters the shorturl, every shorturl must have a path, the path acts as a unique identifer and using this we query the database to find the record. 

If there exists a record then we will send the browser the long URL to the browser to redirect the user to the destination page, the redirection can be temporary or permanent there are trade-offs on using one over the another. Once the browser recieves the it will open the redirected page.

If the record doesn't exists or path is not what was expected then user will get 404 status page.

As simple as that!!!

# Pre-requisites
- NodeJS - [Link](https://nodejs.org)
- PNPM - [Link](https://pnpm.io/installation) (optional)
- Rust - [Link](https://www.rust-lang.org/tools/install)
- Vercel CLI - [Link](https://vercel.com/docs/cli)

# Setup

Open your terminal window, clone the repository at any desired location in your machine, and go inside the project folder using these commands:

```bash
git clone https://github.com/sourabpramanik/shorten-url.git
cd shorten-url
```

## Create a Postgres database
I appreciate Neon and the team behind it, they ingeniously crafted everything around the standard Postgres database so neatly and expanded the scope to use Postgres in different domains without any overhead. Not to mention they also have a humble open-source community.

1. Go to their [website](https://console.neon.tech/), log in, or create an account.

2. Create a project and a database, name them whatever you want:
![Create project GIF](/assets/GIF%20Recording%202024-04-15%20at%206.16.04%20AM.gif)

3. Go to your project dashboard, select your database, and copy the connection string, you will need this connection string later: 
![Get connection string](/assets/GIF%20Recording%202024-04-15%20at%206.18.46%20AM.gif)

## Deploy serverless function Vercel
Deploying projects on [Vercel](https://vercel.com/docs/deployments/overview) is completely free in hobby accounts, and you can add custom domains to your projects if you have however by default Vercel will generate a production URL for your project.

1.  Build the serverless function:
    ```bash
    vercel build --prod --cwd ./shorten-url-functions
    ```
    `--prod` flag will build the function with all the configurations needed by Vercel to deploy and run this function in the production environment.
    
    After the build is completed it generates `.vercel` directory in the root, which has all the directories and configuration files containing lots of information like the runtime, project name, entry point, filesystem mappings, target environment, packages used, commands to be executed and so on. 
    
    If you are curious to know more about it then go ahead and tweak things here and there (at your own risk) as long as everything works as expected.

2. Deploy the build output:
    ```bash
    vercel deploy --prebuilt --prod --cwd ./shorten-url-functions
    ```
    `--prebuilt` flag is for Vercel to locate the `output` directory inside the `.vercel` and deploy it to production

    Only if this is the first time you are deploying this project then you have to answer some of the prompts. This is needed by Vercel to make sure your project is deployed properly wiht any alternate approach you may have.

3. Add environment variables:
    ```
    vercel env add DATABASE_URL --cwd ./shorten-url-functions
    ```
    a prompt will appear asking for the value, paste the database connection string copied from the Neon project dashboard.

    You will need to deploy the function again using the command from step 2 so that the deployed function can use the newly added environment variable, most of the time it is not needed but it's better to make sure

4. That's it with the serverless function deployment, now we need to get the domain. 

    If you have a custom domain and want to use it as the domain for creating short URLs then follow this doc [link](https://vercel.com/docs/projects/domains/add-a-domain) 

    Or else if you want to use the domain generated by Vercel for your project then go to the project dashboard and copy the domain name.

## Install and configure CLI

1. To install the CLI run this command:

    ```bash
    cargo install --path ./shorten-url-cli
    ```
2. Configuring the CLI:
    run the below command and use these when prompted:
    ```bash
    shortenurl config
    ```
    Get these:
    - Postgres connection string from Neon projects dashboard
    - Domain name of the deployed serverless function
    
    And use it when prompted 
    ```bash
    ? Provide postgres connection string:
    ? Provide primary domain configured in Vercel(e.g. foo.com):
    ```

    After the migration completes, your config will be saved in a `shortenurl.toml` file inside the config directory. Look up at these locations depending on your Operation System to locate the config file:

    - Linux
        ```bash
        vim /home/<username>/.config/shortenurl/shortenurl.toml
        ```
    - Windows
        ```bash
        vim C:\Users\<username>\AppData\Roaming\shortenurl\shortenurl.toml
        ```
    - Mac
        ```bash
        vim /Users/<username>/Library/Application Support/shortenurl/shortenurl.toml
        ```
3. That's it, we have finally completed the setup.

# Usage

### Create a short URL
```bash
shortenurl alias create <URL>
```
This will generate an alias for the short URL and creates a record in the database. If the provided URL already exists then it will throw an error.

### Get long and short URLs by alias
```bash
shortenurl alias get <alias>
```
If you have an alias (e.g. U9uaR8C) but are not sure which long URL it belongs to then use this command.

### List all records
```bash
shortenurl alias get-all
```
This command will list out all the short URLs and the respective long URLs

### Remove a record by the alias
```bash
shortenurl alias remove-alias <alias>
```
To remove a short URL and its respective long URL, you can use this command by providing the alias(e.g. U9kHB4Z)

### Remove all records
```bash
shortenurl alias flush
```
Like it says flushes out everything.

> Be careful when you remove a record because you may have used these short URLs at some other places, removing them and using these short URLs again will give 404.

# Conclusion
This was a fun project for me and has lots of scope for new features like analytics, caching, real-time logging, URL grouping by domain name, and so on. But for now, this is it, if you think this project can be used as an alternative to other similar products out there because of the control, minimal to no cost, and hell lot of customizations it provides then let me know [here](shubpramanik241@gmail.com), I would be happy to craft this for production use. 

**Signing out!!!**
