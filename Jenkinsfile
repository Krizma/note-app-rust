pipeline {
  agent any
  stages {
    stage("Verify Cargo Installation") {
      steps {
        sh "cargo --version"
      }
    }
    stage('Build') {
      steps {
        echo "Building..."
        sh 'cargo build'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

  }
}