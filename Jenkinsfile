pipeline {
    environment {
	imageName = 'rinz13r/calculator-app'
	registryCredentialSet = 'dockerhub'
    }
    agent none
    stages {
	stage ('CI') {
	    agent { docker { image 'rust:1.44' } }
	    stages {
		stage ('clone') {
        	    steps {
        	        git 'https://github.com/rinz13r/calculator-app/'
        	    }
        	}
		stage('build') {
        	    steps {
        	        sh 'cargo build'
        	    }
        	}
		stage ('test') {
		    steps {
			sh 'cargo test'
		    }
		}


	    }
	}
	stage ('CD') {
	    stages {
		stage ('Build docker image') {
		    steps {
			script {
			    app = docker.build ("rinz13r/calculator-app:${env.BUILD_ID}")
			}
		    }
		}
		stage ('push image') {
		    steps {
			script {
			    docker.withRegistry ('https://registry.hub.docker.com', 'dockerhub') {
				app.push ("latest")
				app.push ("${env.BUILD_ID}")
			    }
			}
		    }
		}
	    }
	}
    }
}
