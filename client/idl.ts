import { Idl } from "@coral-xyz/anchor";

export const IDL: Idl = {
  "version": "0.1.2",
  "name": "solactivity",
  "instructions": [
    {
      "name": "createProposal",
      "accounts": [
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "group",
          "type": "string"
        },
        {
          "name": "subGroup",
          "type": "string"
        }
      ]
    },
    {
      "name": "migrateProposal",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "author",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "group",
          "type": "string"
        },
        {
          "name": "subGroup",
          "type": "string"
        },
        {
          "name": "score",
          "type": "i32"
        }
      ]
    },
    {
      "name": "deleteProposal",
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "createVote",
      "accounts": [
        {
          "name": "author",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "positive",
          "type": "bool"
        }
      ]
    },
    {
      "name": "changeVote",
      "accounts": [
        {
          "name": "author",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "positive",
          "type": "bool"
        }
      ]
    },
    {
      "name": "deleteVote",
      "accounts": [
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "proposal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vote",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "Proposal",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "program",
            "type": "publicKey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "group",
            "type": "string"
          },
          {
            "name": "subGroup",
            "type": "string"
          },
          {
            "name": "score",
            "type": "i32"
          }
        ]
      }
    },
    {
      "name": "Vote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "publicKey"
          },
          {
            "name": "proposal",
            "type": "publicKey"
          },
          {
            "name": "positive",
            "type": "bool"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NameTooLong",
      "msg": "Name should not exceed 34 characters"
    },
    {
      "code": 6001,
      "name": "GroupTooLong",
      "msg": "Group should not exceed 8 characters"
    },
    {
      "code": 6002,
      "name": "SubGroupTooLong",
      "msg": "Sub Group should not exceed 18 characters"
    },
    {
      "code": 6003,
      "name": "AlreadyUpvoted",
      "msg": "Signer already upvoted this proposal"
    },
    {
      "code": 6004,
      "name": "AlreadyDownvoted",
      "msg": "Signer already downvoted this proposal"
    },
    {
      "code": 6005,
      "name": "NotAuthorOrAdmin",
      "msg": "Signer must be the author or admin"
    },
    {
      "code": 6006,
      "name": "NotAdmin",
      "msg": "Signer must be admin"
    }
  ],
  "metadata": {
    "address": "acTiJkzfuF6vx8Z6GvH4JcZEWCyztU3M5L6BsQDzfNa"
  }
}