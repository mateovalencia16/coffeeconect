rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /departamento/{document} {
      allow create: if false;
      allow read: if false;
      allow write: if false;
      allow delete: if false;
    }

    match /municipio/{document} {
      allow create: if false;
      allow read: if false;
      allow write: if false;
      allow delete: if false;
    }

    match /BDinventario/{document} {
      allow create: if true;
      allow read: if true;
      allow write: if true;
      allow delete: if true;
    }
  }
}
