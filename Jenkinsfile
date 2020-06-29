pipeline {
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
