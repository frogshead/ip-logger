
pipeline {
    agent { node { label 'master' } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build'
            }
        }
    }
}
